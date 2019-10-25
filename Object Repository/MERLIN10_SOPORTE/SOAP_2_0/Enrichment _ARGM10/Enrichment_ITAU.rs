<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Enrichment_ITAU</name>
   <tag></tag>
   <elementGuidId>66356238-3d3a-4578-8c1e-f2199bb59b25</elementGuidId>
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
               &lt;tributaryNumber>20047249393&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
               &lt;!--Optional:-->
               &lt;queryId>26&lt;/queryId>
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
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroTributario&lt;/name>&lt;label>&lt;/label>&lt;value>20047249393&lt;/value>&lt;index>1&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>LIBRETA DE ENROLAMIENTO&lt;/value>&lt;index>2&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NroDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>4724939&lt;/value>&lt;index>3&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Apellidos&lt;/name>&lt;label>&lt;/label>&lt;value>GIAI&lt;/value>&lt;index>4&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nombres&lt;/name>&lt;label>&lt;/label>&lt;value>ELISEO MIGUEL&lt;/value>&lt;index>5&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Genero&lt;/name>&lt;label>&lt;/label>&lt;value>M&lt;/value>&lt;index>6&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaNacimiento&lt;/name>&lt;label>&lt;/label>&lt;value>06/12/1934&lt;/value>&lt;index>7&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NivelSocioEconomico&lt;/name>&lt;label>&lt;/label>&lt;value>D1&lt;/value>&lt;index>8&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>AVDA GRAL INDALECIO CHENAUT&lt;/value>&lt;index>9&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>1838&lt;/value>&lt;index>10&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>PB&lt;/value>&lt;index>11&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>1&lt;/value>&lt;index>12&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>CIUDAD AUTONOMA BUENOS AIRES&lt;/value>&lt;index>13&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>14&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>1426&lt;/value>&lt;index>15&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-34.571011&lt;/value>&lt;index>16&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-58.431754&lt;/value>&lt;index>17&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>18&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Normalizacion&lt;/name>&lt;label>&lt;/label>&lt;value>VA&lt;/value>&lt;index>19&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>PARANA&lt;/value>&lt;index>20&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>21&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>22&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>23&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>VILLA MANTERO&lt;/value>&lt;index>24&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>ENTRE RIOS&lt;/value>&lt;index>25&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>26&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-32.400838&lt;/value>&lt;index>27&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-58.743910&lt;/value>&lt;index>28&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>E&lt;/value>&lt;index>29&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Normalizacion&lt;/name>&lt;label>&lt;/label>&lt;value>CO&lt;/value>&lt;index>30&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>31&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>32&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>33&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>34&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>35&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>36&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>37&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>38&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>39&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>40&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>41&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>42&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>43&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>44&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>45&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>46&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>47&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>48&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>49&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>50&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>51&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>52&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>53&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>54&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>55&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>56&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>57&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>58&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')</verificationScript>
   <wsdlAddress>${Enrichment_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
