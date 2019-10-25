<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>queryLayerByPostalCode</name>
   <tag></tag>
   <elementGuidId>623e488c-903b-4661-8717-bf78400f0872</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.geo.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:queryLayerByPostalCode>
         &lt;!--Optional:-->
         &lt;name>
            &lt;!--Optional:-->
            &lt;clientAccessCode>bb7210305de1b37c21fe7864b148a338&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;zipCode>3500&lt;/zipCode>
         &lt;/name>
      &lt;/soap:queryLayerByPostalCode>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryLayerByPostalCode</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.GEO</defaultValue>
      <description></description>
      <id>539a428a-ec5c-4d3b-b6a0-ae60fdab2a22</id>
      <masked>false</masked>
      <name>GEO</name>
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

WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.customValues.name', 'empresaMusimundo')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.customValues.value', 'Carsa')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.status', 'CO')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.level2', 'CHACO')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.level3', 'SAN FERNANDO')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.level4', 'COLONIA PALMIRA')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.level5', '')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.motive', 'SM')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.outputName', 'empresaMusimundo')
WS.verifyElementText(response, 'queryLayerByPostalCodeResponse.return.zipCode', '3500')</verificationScript>
   <wsdlAddress>${GEO}</wsdlAddress>
</WebServiceRequestEntity>
