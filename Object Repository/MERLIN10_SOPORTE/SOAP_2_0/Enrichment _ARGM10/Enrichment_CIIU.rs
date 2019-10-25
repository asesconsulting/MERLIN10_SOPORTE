<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Enrichment_CIIU</name>
   <tag></tag>
   <elementGuidId>a5f879f2-de7f-4a05-8128-95688e17db4f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.enrichment.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:enrichment>
         &lt;!--Optional:-->
         &lt;enrichment>
            &lt;!--Optional:-->
            &lt;clientAccessCode>a1edeae2a5bd4cde241fdfdb193ca13c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapters> &lt;/customAdapters>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPersonEnrichment>
               &lt;!--Optional:-->
               &lt;level1>AR&lt;/level1>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>27104774755&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
               &lt;!--Optional:-->
               &lt;queryId>9&lt;/queryId>
            &lt;/xPersonEnrichment>
         &lt;/enrichment>
      &lt;/soap:enrichment>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>enrichment</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Enrichment_ARG2</defaultValue>
      <description></description>
      <id>c922b64d-897a-482b-a089-fc0c1cdfbd49</id>
      <masked>false</masked>
      <name>Enrichment_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementText(response, 'enrichmentResponse.return.status', 'EN')
WS.verifyElementText(response, 'enrichmentResponse.return.statusReason', 'SM')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributario&lt;/name>&lt;label>&lt;/label>&lt;value>CUIT PERSONA FISICA&lt;/value>&lt;index>0&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroTributario&lt;/name>&lt;label>&lt;/label>&lt;value>27104774755&lt;/value>&lt;index>1&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>DNI&lt;/value>&lt;index>2&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NroDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>10477475&lt;/value>&lt;index>3&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Apellidos&lt;/name>&lt;label>&lt;/label>&lt;value>VILLAR&lt;/value>&lt;index>4&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nombres&lt;/name>&lt;label>&lt;/label>&lt;value>NELIDA MIRTA&lt;/value>&lt;index>5&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Genero&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>6&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaNacimiento&lt;/name>&lt;label>&lt;/label>&lt;value>29/08/1952&lt;/value>&lt;index>7&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NivelSocioEconomico&lt;/name>&lt;label>&lt;/label>&lt;value>D2&lt;/value>&lt;index>8&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU1&lt;/name>&lt;label>&lt;/label>&lt;value>771290&lt;/value>&lt;index>9&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU2&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>10&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU3&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>11&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU4&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>12&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU5&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>13&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CIIU6&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>14&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>PJE JUAN HIPOLITO VIEYTES&lt;/value>&lt;index>15&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>986&lt;/value>&lt;index>16&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>17&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>18&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>SANTA ROSA&lt;/value>&lt;index>19&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>LA PAMPA&lt;/value>&lt;index>20&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>6300&lt;/value>&lt;index>21&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-36.627357&lt;/value>&lt;index>22&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-64.297049&lt;/value>&lt;index>23&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>24&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>ECUADOR&lt;/value>&lt;index>25&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>923&lt;/value>&lt;index>26&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>27&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>28&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>CIUDAD AUTONOMA BUENOS AIRES&lt;/value>&lt;index>29&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>30&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>1214&lt;/value>&lt;index>31&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-34.599248&lt;/value>&lt;index>32&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-58.405948&lt;/value>&lt;index>33&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>34&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>35&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>49919001&lt;/value>&lt;index>36&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>37&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>38&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>02954&lt;/value>&lt;index>39&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>613824&lt;/value>&lt;index>40&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>41&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>42&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>02954&lt;/value>&lt;index>43&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>438494&lt;/value>&lt;index>44&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>FIJO&lt;/value>&lt;index>45&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>46&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>02954&lt;/value>&lt;index>47&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>660229&lt;/value>&lt;index>48&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>49&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>50&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>51&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>52&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>53&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>54&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>55&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>56&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>57&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>58&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>59&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>60&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>61&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>62&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')</verificationScript>
   <wsdlAddress>${Enrichment_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
