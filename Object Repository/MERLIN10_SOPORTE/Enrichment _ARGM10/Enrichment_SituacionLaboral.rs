<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Enrichment_SituacionLaboral</name>
   <tag></tag>
   <elementGuidId>fa2aa772-987f-414c-89cf-57293aa2124f</elementGuidId>
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
&lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
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
               &lt;tributaryNumber>20217886660&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
               &lt;!--Optional:-->
               &lt;queryId>35&lt;/queryId>
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
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>FISICA&lt;/value>&lt;index>0&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroTributario&lt;/name>&lt;label>&lt;/label>&lt;value>20217886660&lt;/value>&lt;index>1&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributario&lt;/name>&lt;label>&lt;/label>&lt;value>CUIT PERSONA FISICA&lt;/value>&lt;index>2&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributarioCodificado&lt;/name>&lt;label>&lt;/label>&lt;value>80&lt;/value>&lt;index>3&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributarioAfip&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>4&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>21788666&lt;/value>&lt;index>5&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>DNI&lt;/value>&lt;index>6&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumentoCodificado&lt;/name>&lt;label>&lt;/label>&lt;value>96&lt;/value>&lt;index>7&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NombreCompleto&lt;/name>&lt;label>&lt;/label>&lt;value>DENIARD FERNANDO LUIS&lt;/value>&lt;index>8&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Apellidos&lt;/name>&lt;label>&lt;/label>&lt;value>DENIARD&lt;/value>&lt;index>9&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nombres&lt;/name>&lt;label>&lt;/label>&lt;value>FERNANDO LUIS&lt;/value>&lt;index>10&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Genero&lt;/name>&lt;label>&lt;/label>&lt;value>M&lt;/value>&lt;index>11&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaNacimiento&lt;/name>&lt;label>&lt;/label>&lt;value>11/08/1970&lt;/value>&lt;index>12&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Fallecido&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>13&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaFallecimiento&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>14&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EstadoCivil&lt;/name>&lt;label>&lt;/label>&lt;value>C&lt;/value>&lt;index>15&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nacionalidad&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA&lt;/value>&lt;index>16&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Residencia&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA &lt;/value>&lt;index>17&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Votante&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>18&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>DocumentoReemplazo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>19&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>SituacionLaboral&lt;/name>&lt;label>&lt;/label>&lt;value>DEPENDIENTE    &lt;/value>&lt;index>20&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Jubilado&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>21&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CUIT&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>22&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Empresa&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>23&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CantidadEmpleados&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>24&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>AntiguedadLaboral&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>25&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CoberturaMedica&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>26&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')</verificationScript>
   <wsdlAddress>${Enrichment_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
